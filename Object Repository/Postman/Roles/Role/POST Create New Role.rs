<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create New Role</name>
   <tag></tag>
   <elementGuidId>202ddb50-118d-46dd-ab21-804da72751f9</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;ihsan kamil\&quot;,\n  \&quot;desc\&quot;: \&quot;manusia\&quot;,\n  \&quot;managePermission\&quot;: {\n    \&quot;assetAttribute\&quot;: {\n      \&quot;delete\&quot;: true,\n      \&quot;create\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;update\&quot;: false\n    },\n    \&quot;assetPolicies\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;delete\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;update\&quot;: false\n    },\n    \&quot;depreciationGroup\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;delete\&quot;: true,\n      \&quot;update\&quot;: false\n    },\n    \&quot;depreciationMethod\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;update\&quot;: false,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;documentDeletion\&quot;: {\n      \&quot;update\&quot;: false,\n      \&quot;create\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;group\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;update\&quot;: false,\n      \&quot;view\&quot;: false,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;user\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;delete\&quot;: true,\n      \&quot;update\&quot;: false\n    },\n    \&quot;iotReader\&quot;: {\n      \&quot;view\&quot;: false,\n      \&quot;create\&quot;: true,\n      \&quot;update\&quot;: false,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;license\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;update\&quot;: false,\n      \&quot;view\&quot;: false,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;purchaseDocument\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;update\&quot;: false,\n      \&quot;delete\&quot;: true,\n      \&quot;view\&quot;: false\n    },\n    \&quot;purchaseInformation\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;view\&quot;: false,\n      \&quot;update\&quot;: false,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;role\&quot;: {\n      \&quot;view\&quot;: false,\n      \&quot;update\&quot;: false,\n      \&quot;delete\&quot;: true,\n      \&quot;create\&quot;: true\n    },\n    \&quot;tag\&quot;: {\n      \&quot;view\&quot;: false,\n      \&quot;update\&quot;: false,\n      \&quot;create\&quot;: true,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;registerAsset\&quot;: {\n      \&quot;view\&quot;: false,\n      \&quot;create\&quot;: true,\n      \&quot;update\&quot;: false,\n      \&quot;delete\&quot;: true\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6effd902-70a2-45aa-b564-1e9c17819a43</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>2fbb5ed4-9125-48cc-8602-1ebc7f0098ca</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${rolUrl}${rol}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ROLES_BASE_URL</defaultValue>
      <description></description>
      <id>393bef34-a31b-46dd-a976-675b75fd0452</id>
      <masked>false</masked>
      <name>rolUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rolesEndpoint</defaultValue>
      <description></description>
      <id>7d62aa0b-2cea-4f1a-8ed3-01018d3a679b</id>
      <masked>false</masked>
      <name>rol</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>0a7b9c8d-fa3d-4c68-a254-68f2982573fc</id>
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
