<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Routine Task Completion</name>
   <tag></tag>
   <elementGuidId>e345997f-6341-4618-ac41-eee06b715c29</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODNjMTVlNjAxMzBiYzM5YzkyODk1YSIsImlhdCI6MTcxOTkxMDc1MCwiZXhwIjoxNzE5OTUzOTUwfQ.nEIizg1AKAp3SHfJ_qEA_GPa-tE64B41tl6HOgpOIQw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;notes&quot;,
      &quot;value&quot;: &quot;cc&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;maintenanceIndicator&quot;,
      &quot;value&quot;: &quot;[{\&quot;name\&quot;:\&quot;halo\&quot;,\&quot;indicatorType\&quot;:\&quot;Single Value\&quot;,\&quot;dataType\&quot;:\&quot;Number\&quot;,\&quot;isMandatory\&quot;:false,\&quot;isHaveStandardValue\&quot;:false,\&quot;options\&quot;:[],\&quot;standard\&quot;:\&quot;\&quot;,\&quot;measurement\&quot;:\&quot;halo\&quot;}]&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <webElementGuid>e3511da5-1c9c-43fe-b210-1ff72ac4cc59</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODNjMTVlNjAxMzBiYzM5YzkyODk1YSIsImlhdCI6MTcxOTkxMDc1MCwiZXhwIjoxNzE5OTUzOTUwfQ.nEIizg1AKAp3SHfJ_qEA_GPa-tE64B41tl6HOgpOIQw</value>
      <webElementGuid>2576f7b3-279c-4682-85e0-e79a7ba0d81e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${rutUrl}${rutTask}6683b69ffeaba52602c15cb6/completion</restUrl>
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
      <defaultValue>GlobalVariable.routineTask</defaultValue>
      <description></description>
      <id>c78601ac-95a3-45bb-85d9-0d3b7ed74194</id>
      <masked>false</masked>
      <name>rutTask</name>
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
