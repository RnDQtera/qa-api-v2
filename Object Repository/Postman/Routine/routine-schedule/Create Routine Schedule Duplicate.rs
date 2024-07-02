<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Routine Schedule Duplicate</name>
   <tag></tag>
   <elementGuidId>f6742443-084a-488e-913d-a01aa412fce1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bt}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;taskName\&quot;:\&quot;gacoba\&quot;,\n  \&quot;nextSchedule\&quot;:1719939600000,\n  \&quot;frequency\&quot;:{\n    \&quot;label\&quot;:\&quot;Daily\&quot;\n  },\n  \&quot;duration\&quot;:2,\n  \&quot;maintenanceIndicator\&quot;:\&quot;[{\\\&quot;name\\\&quot;:\\\&quot;indiekator\\\&quot;,\\\&quot;indicatorType\\\&quot;:\\\&quot;Single Value\\\&quot;,\\\&quot;dataType\\\&quot;:\\\&quot;Number\\\&quot;,\\\&quot;isMandatory\\\&quot;:false,\\\&quot;isHaveStandardValue\\\&quot;:false,\\\&quot;standard\\\&quot;:\\\&quot;\\\&quot;,\\\&quot;measurement\\\&quot;:\\\&quot;garis\\\&quot;}]\&quot;,\n  \&quot;assetNames\&quot;:[{\n    \&quot;_id\&quot;:\&quot;65f36bb35a5072e82eb557c9\&quot;,\n    \&quot;group\&quot;:\&quot;65fcf21286bbf6430e72ac61\&quot;}]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2b737b55-042a-4cdb-b89c-d9714583e677</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>c4ec47b2-cb0b-4def-9e28-9d9f696d8450</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${rutUrl}${rutSch}duplicate/6683a8f3feaba52602c15640</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ROUTINE_BASE_URL</defaultValue>
      <description></description>
      <id>5bf8edb0-83a9-46e6-9d18-28edeea80879</id>
      <masked>false</masked>
      <name>rutUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.routineSchedule</defaultValue>
      <description></description>
      <id>c78601ac-95a3-45bb-85d9-0d3b7ed74194</id>
      <masked>false</masked>
      <name>rutSch</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>b7a431aa-819f-4400-b64f-7a7a6524b05c</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
